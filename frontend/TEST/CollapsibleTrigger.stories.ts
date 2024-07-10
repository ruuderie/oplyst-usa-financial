import type { Meta, StoryObj } from '@storybook/vue3';

import CollapsibleTrigger from '../components/ui/collapsible/CollapsibleTrigger.vue';

const meta = {
  title: 'CollapsibleTrigger',
  component: CollapsibleTrigger,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof CollapsibleTrigger>;

export default meta;
type Story = StoryObj<typeof CollapsibleTrigger>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};