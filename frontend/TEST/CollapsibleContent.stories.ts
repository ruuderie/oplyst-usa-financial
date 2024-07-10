import type { Meta, StoryObj } from '@storybook/vue3';

import CollapsibleContent from '../components/ui/collapsible/CollapsibleContent.vue';

const meta = {
  title: 'CollapsibleContent',
  component: CollapsibleContent,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof CollapsibleContent>;

export default meta;
type Story = StoryObj<typeof CollapsibleContent>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};