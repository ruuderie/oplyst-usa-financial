import type { Meta, StoryObj } from '@storybook/vue3';

import TooltipContent from '../components/ui/tooltip/TooltipContent.vue';

const meta = {
  title: 'TooltipContent',
  component: TooltipContent,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof TooltipContent>;

export default meta;
type Story = StoryObj<typeof TooltipContent>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};