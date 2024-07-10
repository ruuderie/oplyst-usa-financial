import type { Meta, StoryObj } from '@storybook/vue3';

import TooltipTrigger from '../components/ui/tooltip/TooltipTrigger.vue';

const meta = {
  title: 'TooltipTrigger',
  component: TooltipTrigger,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof TooltipTrigger>;

export default meta;
type Story = StoryObj<typeof TooltipTrigger>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};