import type { Meta, StoryObj } from '@storybook/vue3';

import Collapsible from '../components/ui/collapsible/Collapsible.vue';

const meta = {
  title: 'Collapsible',
  component: Collapsible,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof Collapsible>;

export default meta;
type Story = StoryObj<typeof Collapsible>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};